// Generated macro for ChildrenBuilder (struct)
macro_rules! Depcrate_executor_look_aheadChildrenBuilder {
() => {
// Module: crate::executor::look_ahead
// Provides: {"ChildrenBuilder"}
// Dependencies: {}
struct ChildrenBuilder < 'a , 'f , S > { vars : & 'a Variables < S > , fragments : & 'a HashMap < & 'a str , Fragment < 'a , S > > , type_filter : Applies < 'f > , output : Vec < LookAheadSelection < 'a , S > > , }
};
}
