// Generated macro for args_impl (macro)
macro_rules! Depcrate___macros_convertargs_impl {
() => {
// Module: crate::__macros::convert
// Provides: {"args_impl"}
// Dependencies: {}
macro_rules ! args_impl { ($ ($ a : ident : $ t : ident) ,*) => (impl <$ ($ t : ConvertArgument) ,*> ConvertArguments for ($ ($ t ,) *) { type __Inner = ($ ($ t :: __Inner ,) *) ; type __WritebackOnDrop = ($ ($ t :: __WritebackOnDrop ,) *) ; # [inline] unsafe fn __into_arguments (self) -> (Self :: __Inner , Self :: __WritebackOnDrop) { let ($ ($ a ,) *) = self ; $ (let $ a = unsafe { ConvertArgument :: __into_argument ($ a) } ;) * (($ ($ a . 0 ,) *) , ($ ($ a . 1 ,) *)) } } impl <$ ($ t ,) * T > TupleExtender < T > for ($ ($ t ,) *) { type PlusOneArgument = ($ ($ t ,) * T ,) ; # [inline] fn add_argument (self , arg : T) -> Self :: PlusOneArgument { let ($ ($ a ,) *) = self ; ($ ($ a ,) * arg ,) } }) ; }
};
}
