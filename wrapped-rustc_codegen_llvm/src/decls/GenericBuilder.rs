macro_rules! deps {
    () => {
        Builder!();
        SCx!();
        GenericCx!();
    };
}

macro_rules! GenericBuilder {
    () => {
        deps!();
        # [must_use] pub (crate) struct GenericBuilder < 'a , 'll , CX : Borrow < SCx < 'll > > > { pub llbuilder : & 'll mut llvm :: Builder < 'll > , pub cx : & 'a GenericCx < 'll , CX > , }
    };
}

GenericBuilder!()