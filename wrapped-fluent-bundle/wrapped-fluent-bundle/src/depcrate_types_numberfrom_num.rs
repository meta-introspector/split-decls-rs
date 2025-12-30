// Generated macro for from_num (macro)
macro_rules! Depcrate_types_numberfrom_num {
() => {
// Module: crate::types::number
// Provides: {"from_num"}
// Dependencies: {}
macro_rules ! from_num { ($ num : ty) => { impl From <$ num > for FluentNumber { fn from (n : $ num) -> Self { Self { value : n as f64 , options : FluentNumberOptions :: default () , } } } impl From <&$ num > for FluentNumber { fn from (n : &$ num) -> Self { Self { value : * n as f64 , options : FluentNumberOptions :: default () , } } } impl From < FluentNumber > for $ num { fn from (input : FluentNumber) -> Self { input . value as $ num } } impl From <& FluentNumber > for $ num { fn from (input : & FluentNumber) -> Self { input . value as $ num } } impl From <$ num > for FluentValue <'_ > { fn from (n : $ num) -> Self { FluentValue :: Number (n . into ()) } } impl From <&$ num > for FluentValue <'_ > { fn from (n : &$ num) -> Self { FluentValue :: Number (n . into ()) } } } ; ($ ($ num : ty) +) => { $ (from_num ! ($ num) ;) + } ; }
};
}
