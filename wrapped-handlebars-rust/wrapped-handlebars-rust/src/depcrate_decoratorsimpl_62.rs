// Generated macro for impl_62 (impl)
macro_rules! Depcrate_decoratorsimpl_62 {
() => {
// Module: crate::decorators
// Provides: {"impl_62"}
// Dependencies: {}
# [doc = " Implement `DecoratorDef` for bare function so we can use function as decorator"] impl < F : for < 'reg , 'rc > Fn (& Decorator < 'rc > , & 'reg Registry < 'reg > , & 'rc Context , & mut RenderContext < 'reg , 'rc > ,) -> DecoratorResult , > DecoratorDef for F { fn call < 'reg : 'rc , 'rc > (& 'reg self , d : & Decorator < 'rc > , reg : & 'reg Registry < 'reg > , ctx : & 'rc Context , rc : & mut RenderContext < 'reg , 'rc > ,) -> DecoratorResult { (* self) (d , reg , ctx , rc) } }
};
}
