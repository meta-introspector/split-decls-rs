// Generated macro for impl_438 (impl)
macro_rules! Depcrate_userdataimpl_438 {
() => {
// Module: crate::userdata
// Provides: {"impl_438"}
// Dependencies: {}
impl UserdataGuard { fn new (u : * mut c_void) -> Self { UserdataGuard { data : Some (Userdata { userdata : u , # [cfg (not (feature = "no_log_capture"))] log_callback : None , }) , } } # [doc = " Even though we have a Drop impl on this guard, when possible it's"] # [doc = " best to call try_drop explicitly. That way any failures of internal"] # [doc = " variants can be signaled to the user immediately by returning"] # [doc = " rustls_result::Panic."] pub (crate) fn try_drop (mut self) -> Result < () , UserdataError > { self . try_pop () } fn try_pop (& mut self) -> Result < () , UserdataError > { let expected_data = self . data . as_ref () . ok_or (UserdataError :: AlreadyPopped) ? . userdata ; USERDATA . try_with (| userdata | { userdata . try_borrow_mut () . map_or_else (| _ | Err (UserdataError :: AlreadyBorrowed) , | mut v | { let u = v . pop () . ok_or (UserdataError :: EmptyStack) ? ; self . data = None ; if ptr :: eq (u . userdata , expected_data) { Ok (()) } else { Err (UserdataError :: WrongData) } } ,) }) . unwrap_or (Err (UserdataError :: AccessError)) } }
};
}
