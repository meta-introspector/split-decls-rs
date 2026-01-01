// SRC: ../rust/compiler/rustc_mir_transform/src/required_consts.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */
use crate::rustc_complete::mir::visit::Visitor;
use crate::rustc_complete::mir::{Body, ConstOperand, Location, traversal};
/* AST_META: AST_ID=2 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=4 */

pub(super) struct RequiredConstsVisitor<'tcx> {
    required_consts: Vec<ConstOperand<'tcx>>,
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=7 | LINES=10 */

impl<'tcx> RequiredConstsVisitor<'tcx> {
    pub(super) fn compute_required_consts(body: &mut Body<'tcx>) {
        let mut visitor = RequiredConstsVisitor { required_consts: Vec::new() };
        for (bb, bb_data) in traversal::reverse_postorder(&body) {
            visitor.visit_basic_block_data(bb, bb_data);
        }
        body.set_required_consts(visitor.required_consts);
    }
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=visit_const_operand | COMPLEXITY=8 | LINES=8 */

impl<'tcx> Visitor<'tcx> for RequiredConstsVisitor<'tcx> {
    fn visit_const_operand(&mut self, constant: &ConstOperand<'tcx>, _: Location) {
        if constant.const_.is_required_const() {
            self.required_consts.push(*constant);
        }
    }
}