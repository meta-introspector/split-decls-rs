macro_rules! deps {
    () => {
        SmallVec!();
        SBuilder!();
        Funclet!();
    };
}

macro_rules! impl_159 {
    () => {
        deps!();
        impl < 'a , 'll > SBuilder < 'a , 'll > { pub (crate) fn call (& mut self , llty : & 'll Type , llfn : & 'll Value , args : & [& 'll Value] , funclet : Option < & Funclet < 'll > > ,) -> & 'll Value { debug ! ("call {:?} with args ({:?})" , llfn , args) ; let args = self . check_call ("call" , llty , llfn , args) ; let funclet_bundle = funclet . map (| funclet | funclet . bundle ()) ; let mut bundles : SmallVec < [_ ; 2] > = SmallVec :: new () ; if let Some (funclet_bundle) = funclet_bundle { bundles . push (funclet_bundle) ; } let call = unsafe { llvm :: LLVMBuildCallWithOperandBundles (self . llbuilder , llty , llfn , args . as_ptr () as * const & llvm :: Value , args . len () as c_uint , bundles . as_ptr () , bundles . len () as c_uint , c"" . as_ptr () ,) } ; call } }
    };
}

impl_159!();