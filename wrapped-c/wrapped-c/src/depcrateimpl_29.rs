// Generated macro for impl_29 (impl)
macro_rules! Depcrateimpl_29 {
() => {
// Module: crate
// Provides: {"impl_29"}
// Dependencies: {}
impl C { fn interface < 'a > (& 'a mut self , resolve : & 'a Resolve , in_import : bool , wasm_import_module : Option < & 'a str > ,) -> InterfaceGenerator < 'a > { InterfaceGenerator { src : Source :: default () , r#gen : self , resolve , interface : None , in_import , wasm_import_module , } } fn h_include (& mut self , s : & str) { self . h_includes . push (s . to_string ()) ; } fn c_include (& mut self , s : & str) { self . c_includes . push (s . to_string ()) ; } fn char_type (& self) -> & 'static str { match self . opts . string_encoding { StringEncoding :: UTF8 => "uint8_t" , StringEncoding :: UTF16 => "uint16_t" , StringEncoding :: CompactUTF16 => panic ! ("Compact UTF16 unsupported") , } } fn type_name (& mut self , ty : & Type) -> String { let mut name = String :: new () ; self . push_type_name (ty , & mut name) ; name } fn push_type_name (& mut self , ty : & Type , dst : & mut String) { match ty { Type :: Bool => dst . push_str ("bool") , Type :: Char => dst . push_str ("uint32_t") , Type :: U8 => dst . push_str ("uint8_t") , Type :: S8 => dst . push_str ("int8_t") , Type :: U16 => dst . push_str ("uint16_t") , Type :: S16 => dst . push_str ("int16_t") , Type :: U32 => dst . push_str ("uint32_t") , Type :: S32 => dst . push_str ("int32_t") , Type :: U64 => dst . push_str ("uint64_t") , Type :: S64 => dst . push_str ("int64_t") , Type :: F32 => dst . push_str ("float") , Type :: F64 => dst . push_str ("double") , Type :: String => { dst . push_str (& self . world . to_snake_case ()) ; dst . push_str ("_") ; dst . push_str ("string_t") ; self . needs_string = true ; } Type :: ErrorContext => dst . push_str ("error_context") , Type :: Id (id) => { if let Some (name) = self . type_names . get (id) { dst . push_str (name) ; return ; } panic ! ("failed to find type name for {id:?}") ; } } } # [doc = " Removes all types from `self.{dtor_funcs,type_names,resources}` which"] # [doc = " are redefined in exports."] # [doc = ""] # [doc = " WIT interfaces can be both imported and exported but they're represented"] # [doc = " with the same `TypeId` internally within the `wit-parser`"] # [doc = " representation. This means that duplicate types need to be generated for"] # [doc = " exports, even if the same interface was already imported. If nothing"] # [doc = " were done here though then the same type imported and exported wouldn't"] # [doc = " generate anything new since preexisting types are skipped in"] # [doc = " `define_live_types`."] # [doc = ""] # [doc = " This function will trim the sets on `self` to only retain those types"] # [doc = " which exports refer to that come from imports."] fn remove_types_redefined_by_exports (& mut self , resolve : & Resolve , world : WorldId) { let live_import_types = imported_types_used_by_exported_interfaces (resolve , world) ; self . dtor_funcs . retain (| k , _ | live_import_types . contains (k)) ; self . type_names . retain (| k , _ | live_import_types . contains (k)) ; self . resources . retain (| k , _ | live_import_types . contains (k)) ; } fn perform_cast (& mut self , op : & str , cast : & Bitcast) -> String { match cast { Bitcast :: I32ToF32 | Bitcast :: I64ToF32 => { self . needs_union_int32_float = true ; format ! ("((union int32_float){{ (int32_t) {} }}).b" , op) } Bitcast :: F32ToI32 | Bitcast :: F32ToI64 => { self . needs_union_float_int32 = true ; format ! ("((union float_int32){{ {} }}).b" , op) } Bitcast :: I64ToF64 => { self . needs_union_int64_double = true ; format ! ("((union int64_double){{ (int64_t) {} }}).b" , op) } Bitcast :: F64ToI64 => { self . needs_union_double_int64 = true ; format ! ("((union double_int64){{ {} }}).b" , op) } Bitcast :: I32ToI64 | Bitcast :: LToI64 | Bitcast :: PToP64 => { format ! ("(int64_t) {}" , op) } Bitcast :: I64ToI32 | Bitcast :: I64ToL => { format ! ("(int32_t) {}" , op) } Bitcast :: I64ToP64 | Bitcast :: P64ToI64 => { format ! ("{}" , op) } Bitcast :: P64ToP | Bitcast :: I32ToP | Bitcast :: LToP => { format ! ("(uint8_t *) {}" , op) } Bitcast :: PToI32 | Bitcast :: PToL => format ! ("(uintptr_t) {}" , op) , Bitcast :: I32ToL | Bitcast :: LToI32 | Bitcast :: None => op . to_string () , Bitcast :: Sequence (sequence) => { let [first , second] = & * * sequence ; let inner = self . perform_cast (op , first) ; self . perform_cast (& inner , second) } } } fn generate_async_helpers (& mut self) { let snake = self . world . to_snake_case () ; let shouty = self . world . to_shouty_snake_case () ; uwriteln ! (self . src . h_async , "
typedef uint32_t {snake}_subtask_status_t;
typedef uint32_t {snake}_subtask_t;
#define {shouty}_SUBTASK_STATE(status) (({snake}_subtask_state_t) ((status) & 0xf))
#define {shouty}_SUBTASK_HANDLE(status) (({snake}_subtask_t) ((status) >> 4))

typedef enum {snake}_subtask_state {{
    {shouty}_SUBTASK_STARTING,
    {shouty}_SUBTASK_STARTED,
    {shouty}_SUBTASK_RETURNED,
    {shouty}_SUBTASK_STARTED_CANCELLED,
    {shouty}_SUBTASK_RETURNED_CANCELLED,
}} {snake}_subtask_state_t;

{snake}_subtask_status_t {snake}_subtask_cancel({snake}_subtask_t subtask);
void {snake}_subtask_drop({snake}_subtask_t subtask);

typedef uint32_t {snake}_callback_code_t;
#define {shouty}_CALLBACK_CODE_EXIT 0
#define {shouty}_CALLBACK_CODE_YIELD 1
#define {shouty}_CALLBACK_CODE_WAIT(set) (2 | (set << 4))
#define {shouty}_CALLBACK_CODE_POLL(set) (3 | (set << 4))

typedef enum {snake}_event_code {{
    {shouty}_EVENT_NONE,
    {shouty}_EVENT_SUBTASK,
    {shouty}_EVENT_STREAM_READ,
    {shouty}_EVENT_STREAM_WRITE,
    {shouty}_EVENT_FUTURE_READ,
    {shouty}_EVENT_FUTURE_WRITE,
    {shouty}_EVENT_CANCEL,
}} {snake}_event_code_t;

typedef struct {snake}_event {{
    {snake}_event_code_t event;
    uint32_t waitable;
    uint32_t code;
}} {snake}_event_t;

typedef uint32_t {snake}_waitable_set_t;
{snake}_waitable_set_t {snake}_waitable_set_new(void);
void {snake}_waitable_join(uint32_t waitable, {snake}_waitable_set_t set);
void {snake}_waitable_set_drop({snake}_waitable_set_t set);
void {snake}_waitable_set_wait({snake}_waitable_set_t set, {snake}_event_t *event);
void {snake}_waitable_set_poll({snake}_waitable_set_t set, {snake}_event_t *event);

void {snake}_task_cancel(void);

typedef uint32_t {snake}_waitable_status_t;
#define {shouty}_WAITABLE_STATE(status) (({snake}_waitable_state_t) ((status) & 0xf))
#define {shouty}_WAITABLE_COUNT(status) ((uint32_t) ((status) >> 4))
#define {shouty}_WAITABLE_STATUS_BLOCKED (({snake}_waitable_status_t) -1)

typedef enum {snake}_waitable_state {{
    {shouty}_WAITABLE_COMPLETED,
    {shouty}_WAITABLE_DROPPED,
    {shouty}_WAITABLE_CANCELLED,
}} {snake}_waitable_state_t;

void {snake}_backpressure_set(bool enable);
void {snake}_backpressure_inc(void);
void {snake}_backpressure_dec(void);
void* {snake}_context_get(void);
void {snake}_context_set(void*);
void {snake}_yield(void);
uint32_t {snake}_yield_cancellable(void);
            ") ; uwriteln ! (self . src . c_async , r#"
__attribute__((__import_module__("$root"), __import_name__("[subtask-cancel]")))
extern uint32_t __subtask_cancel(uint32_t handle);

{snake}_subtask_status_t {snake}_subtask_cancel({snake}_subtask_t subtask) {{
    return __subtask_cancel(subtask);
}}

__attribute__((__import_module__("$root"), __import_name__("[subtask-drop]")))
extern void __subtask_drop(uint32_t handle);

void {snake}_subtask_drop({snake}_subtask_t subtask) {{
    __subtask_drop(subtask);
}}

__attribute__((__import_module__("$root"), __import_name__("[waitable-set-new]")))
extern uint32_t __waitable_set_new(void);

{snake}_waitable_set_t {snake}_waitable_set_new(void) {{
    return __waitable_set_new();
}}

__attribute__((__import_module__("$root"), __import_name__("[waitable-join]")))
extern void __waitable_join(uint32_t, uint32_t);

void {snake}_waitable_join(uint32_t waitable, {snake}_waitable_set_t set) {{
    __waitable_join(waitable, set);
}}

__attribute__((__import_module__("$root"), __import_name__("[waitable-set-drop]")))
extern void __waitable_set_drop(uint32_t);

void {snake}_waitable_set_drop({snake}_waitable_set_t set) {{
    __waitable_set_drop(set);
}}

__attribute__((__import_module__("$root"), __import_name__("[waitable-set-wait]")))
extern uint32_t __waitable_set_wait(uint32_t, uint32_t*);
__attribute__((__import_module__("$root"), __import_name__("[waitable-set-poll]")))
extern uint32_t __waitable_set_poll(uint32_t, uint32_t*);

void {snake}_waitable_set_wait({snake}_waitable_set_t set, {snake}_event_t *event) {{
    event->event = ({snake}_event_code_t) __waitable_set_wait(set, &event->waitable);
}}

void {snake}_waitable_set_poll({snake}_waitable_set_t set, {snake}_event_t *event) {{
    event->event = ({snake}_event_code_t) __waitable_set_poll(set, &event->waitable);
}}

__attribute__((__import_module__("[export]$root"), __import_name__("[task-cancel]")))
extern void __task_cancel(void);

void {snake}_task_cancel() {{
    __task_cancel();
}}

__attribute__((__import_module__("$root"), __import_name__("[backpressure-set]")))
extern void __backpressure_set(bool enable);

void {snake}_backpressure_set(bool enable) {{
    __backpressure_set(enable);
}}

__attribute__((__import_module__("$root"), __import_name__("[backpressure-inc]")))
extern void __backpressure_inc(void);

void {snake}_backpressure_inc(void) {{
    __backpressure_inc();
}}

__attribute__((__import_module__("$root"), __import_name__("[backpressure-dec]")))
extern void __backpressure_dec(void);

void {snake}_backpressure_dec(void) {{
    __backpressure_dec();
}}

__attribute__((__import_module__("$root"), __import_name__("[context-get-0]")))
extern void* __context_get(void);

void* {snake}_context_get() {{
    return __context_get();
}}

__attribute__((__import_module__("$root"), __import_name__("[context-set-0]")))
extern void __context_set(void*);

void {snake}_context_set(void *val) {{
    return __context_set(val);
}}

__attribute__((__import_module__("$root"), __import_name__("[thread-yield]")))
extern uint32_t __thread_yield(void);

void {snake}_yield(void) {{
    __thread_yield();
}}

__attribute__((__import_module__("$root"), __import_name__("[cancellable][thread-yield]")))
extern uint32_t __thread_yield_cancellable(void);

uint32_t {snake}_yield_cancellable(void) {{
    return __thread_yield_cancellable();
}}
            "#) ; } }
};
}
