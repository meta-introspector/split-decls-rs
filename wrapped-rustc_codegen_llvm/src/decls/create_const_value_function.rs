macro_rules! deps {
    () => {
        AttributePlace!();
        SBuilder!();
        Visibility!();
        CallConv!();
        UnnamedAddr!();
        AttributeKind!();
        SimpleCx!();
    };
}

macro_rules! create_const_value_function {
    () => {
        deps!();
        fn create_const_value_function (tcx : TyCtxt < '_ > , cx : & SimpleCx < '_ > , name : & str , output : & Type , value : & Value ,) { let ty = cx . type_func (& [] , output) ; let llfn = declare_simple_fn (& cx , name , llvm :: CallConv :: CCallConv , llvm :: UnnamedAddr :: Global , llvm :: Visibility :: from_generic (tcx . sess . default_visibility ()) , ty ,) ; attributes :: apply_to_llfn (llfn , llvm :: AttributePlace :: Function , & [llvm :: AttributeKind :: AlwaysInline . create_attr (cx . llcx)] ,) ; let llbb = unsafe { llvm :: LLVMAppendBasicBlockInContext (cx . llcx , llfn , c"entry" . as_ptr ()) } ; let mut bx = SBuilder :: build (& cx , llbb) ; bx . ret (value) ; }
    };
}

create_const_value_function!()