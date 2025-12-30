// Generated macro for impl_582 (impl)
macro_rules! Depcrate_exceptionimpl_582 {
() => {
// Module: crate::exception
// Provides: {"impl_582"}
// Dependencies: {}
impl Exception { fn is_nsexception (& self) -> Option < bool > { if self . class () . responds_to (sel ! (isKindOfClass :)) { let obj : * const Exception = self ; let obj = unsafe { obj . cast :: < NSObject > () . as_ref () . unwrap () } ; let name = CStr :: from_bytes_with_nul (b"NSException\0") . unwrap () ; Some (obj . isKindOfClass (AnyClass :: get (name) ?)) } else { Some (false) } } # [cfg (feature = "catch-all")] pub (crate) fn stack_trace (& self) -> impl fmt :: Display + '_ { struct Helper < 'a > (& 'a Exception) ; impl fmt :: Display for Helper < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if let Some (true) = self . 0 . is_nsexception () { autoreleasepool_leaking (| pool | { let call_stack_symbols : Option < Retained < NSObject > > = unsafe { msg_send ! [self . 0 , callStackSymbols] } ; if let Some (call_stack_symbols) = call_stack_symbols { writeln ! (f , "stack backtrace:") ? ; let count : NSUInteger = unsafe { msg_send ! [& call_stack_symbols , count] } ; let mut i = 0 ; while i < count { let symbol : Retained < NSObject > = unsafe { msg_send ! [& call_stack_symbols , objectAtIndex : i] } ; let symbol = unsafe { nsstring_to_str (& symbol , pool) } ; writeln ! (f , "{symbol}") ? ; i += 1 ; } } Ok (()) }) } else { Ok (()) } } } Helper (self) } }
};
}
