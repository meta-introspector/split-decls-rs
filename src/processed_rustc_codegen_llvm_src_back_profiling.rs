// SRC: ../rust/compiler/rustc_codegen_llvm/src/back/profiling.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use std::ffi::{CStr, c_void};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=5 */
use std::os::raw::c_char;
use std::sync::Arc;

use measureme::event_id::SEPARATOR_BYTE;
use measureme::{EventId, StringComponent, StringId};
/* AST_META: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use crate::rustc_data_structures::profiling::{SelfProfiler, TimingGuard};
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=llvm_args_to_string_id | COMPLEXITY=6 | LINES=15 */

fn llvm_args_to_string_id(profiler: &SelfProfiler, pass_name: &str, ir_name: &str) -> EventId {
    let pass_name = profiler.get_or_alloc_cached_string(pass_name);
    let mut components = vec![StringComponent::Ref(pass_name)];
    // handle that LazyCallGraph::SCC is a comma separated list within parentheses
    let parentheses: &[_] = &['(', ')'];
    let trimmed = ir_name.trim_matches(parentheses);
    for part in trimmed.split(", ") {
        let demangled_ir_name = rustc_demangle::demangle(part).to_string();
        let ir_name = profiler.get_or_alloc_cached_string(demangled_ir_name);
        components.push(StringComponent::Value(SEPARATOR_BYTE));
        components.push(StringComponent::Ref(ir_name));
    }
    EventId::from_label(profiler.alloc_string(components.as_slice()))
}
/* AST_META: AST_ID=5 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=6 */

pub(crate) struct LlvmSelfProfiler<'a> {
    profiler: Arc<SelfProfiler>,
    stack: Vec<TimingGuard<'a>>,
    llvm_pass_event_kind: StringId,
}
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=before_pass_callback | COMPLEXITY=7 | LINES=16 */

impl<'a> LlvmSelfProfiler<'a> {
    pub(crate) fn new(profiler: Arc<SelfProfiler>) -> Self {
        let llvm_pass_event_kind = profiler.alloc_string("LLVM Pass");
        Self { profiler, stack: Vec::default(), llvm_pass_event_kind }
    }

    fn before_pass_callback(&'a mut self, pass_name: &str, ir_name: &str) {
        let event_id = llvm_args_to_string_id(&self.profiler, pass_name, ir_name);

        self.stack.push(TimingGuard::start(&self.profiler, self.llvm_pass_event_kind, event_id));
    }
    fn after_pass_callback(&mut self) {
        self.stack.pop();
    }
}
/* AST_META: AST_ID=7 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=12 | LINES=13 */

pub(crate) unsafe extern "C" fn selfprofile_before_pass_callback(
    llvm_self_profiler: *mut c_void,
    pass_name: *const c_char,
    ir_name: *const c_char,
) {
    unsafe {
        let llvm_self_profiler = &mut *(llvm_self_profiler as *mut LlvmSelfProfiler<'_>);
        let pass_name = CStr::from_ptr(pass_name).to_str().expect("valid UTF-8");
        let ir_name = CStr::from_ptr(ir_name).to_str().expect("valid UTF-8");
        llvm_self_profiler.before_pass_callback(pass_name, ir_name);
    }
}
/* AST_META: AST_ID=8 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=11 | LINES=5 */

pub(crate) unsafe extern "C" fn selfprofile_after_pass_callback(llvm_self_profiler: *mut c_void) {
    let llvm_self_profiler = unsafe { &mut *(llvm_self_profiler as *mut LlvmSelfProfiler<'_>) };
    llvm_self_profiler.after_pass_callback();
}