macro_rules! deps {
    () => {
        Odb!();
    };
}

macro_rules! Mempack {
    () => {
        deps!();
        # [doc = " A structure to represent a mempack backend for the object database. The"] # [doc = " Mempack is bound to the Odb that it was created from, and cannot outlive"] # [doc = " that Odb."] pub struct Mempack < 'odb > { raw : * mut raw :: git_odb_backend , _marker : marker :: PhantomData < & 'odb Odb < 'odb > > , }
    };
}

Mempack!()