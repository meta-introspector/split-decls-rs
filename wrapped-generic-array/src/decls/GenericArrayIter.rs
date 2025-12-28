macro_rules! deps {
    () => {
        ArrayLength!();
        GenericArray!();
    };
}

macro_rules! GenericArrayIter {
    () => {
        deps!();
        # [doc = " An iterator that moves out of a [`GenericArray`]"] pub struct GenericArrayIter < T , N : ArrayLength > { array : ManuallyDrop < GenericArray < T , N > > , index : usize , index_back : usize , }
    };
}

GenericArrayIter!();