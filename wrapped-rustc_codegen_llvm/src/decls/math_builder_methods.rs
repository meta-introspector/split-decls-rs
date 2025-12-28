macro_rules! math_builder_methods {
    () => {
        macro_rules ! math_builder_methods { ($ ($ name : ident ($ ($ arg : ident) ,*) => $ llvm_capi : ident) ,+ $ (,) ?) => { $ (fn $ name (& mut self , $ ($ arg : &'ll Value) ,*) -> &'ll Value { unsafe { llvm ::$ llvm_capi (self . llbuilder , $ ($ arg ,) * UNNAMED) } }) + } }
    };
}

math_builder_methods!()