// Generated macro for delegate (macro)
macro_rules! Depcrate_bndelegate {
() => {
// Module: crate::bn
// Provides: {"delegate"}
// Dependencies: {}
macro_rules ! delegate { ($ t : ident , $ m : ident) => { impl <'a , 'b > $ t <&'b BigNum > for &'a BigNumRef { type Output = BigNum ; fn $ m (self , oth : & BigNum) -> BigNum { $ t ::$ m (self , oth . deref ()) } } impl <'a , 'b > $ t <&'b BigNumRef > for &'a BigNum { type Output = BigNum ; fn $ m (self , oth : & BigNumRef) -> BigNum { $ t ::$ m (self . deref () , oth) } } impl <'a , 'b > $ t <&'b BigNum > for &'a BigNum { type Output = BigNum ; fn $ m (self , oth : & BigNum) -> BigNum { $ t ::$ m (self . deref () , oth . deref ()) } } } ; }
};
}
