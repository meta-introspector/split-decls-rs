// Generated macro for impl_writeable_for_each_subtag_str_no_test (macro)
macro_rules! Depcrate_helpersimpl_writeable_for_each_subtag_str_no_test {
() => {
// Module: crate::helpers
// Provides: {"impl_writeable_for_each_subtag_str_no_test"}
// Dependencies: {}
# [macro_export] # [doc (hidden)] macro_rules ! impl_writeable_for_each_subtag_str_no_test { ($ type : tt $ (, $ self : ident , $ borrow_cond : expr => $ borrow : expr) ?) => { impl writeable :: Writeable for $ type { fn write_to < W : core :: fmt :: Write + ? Sized > (& self , sink : & mut W) -> core :: fmt :: Result { let mut initial = true ; self . for_each_subtag_str (& mut | subtag | { if initial { initial = false ; } else { sink . write_char ('-') ?; } sink . write_str (subtag) }) } # [inline] fn writeable_length_hint (& self) -> writeable :: LengthHint { let mut result = writeable :: LengthHint :: exact (0) ; let mut initial = true ; self . for_each_subtag_str ::< core :: convert :: Infallible , _ > (& mut | subtag | { if initial { initial = false ; } else { result += 1 ; } result += subtag . len () ; Ok (()) }) . expect ("infallible") ; result } $ (fn writeable_borrow (& self) -> Option <& str > { let $ self = self ; if $ borrow_cond { $ borrow } else { None } }) ? } writeable :: impl_display_with_writeable ! ($ type , # [cfg (feature = "alloc")]) ; } ; }
};
}
