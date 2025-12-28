macro_rules! deps {
    () => {
        Endian!();
        Bytes!();
    };
}

macro_rules! LoadCommandData {
    () => {
        deps!();
        # [doc = " The data for a [`macho::LoadCommand`]."] # [derive (Debug , Clone , Copy)] pub struct LoadCommandData < 'data , E : Endian > { cmd : u32 , data : Bytes < 'data > , marker : PhantomData < E > , }
    };
}

LoadCommandData!()