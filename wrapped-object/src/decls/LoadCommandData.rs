macro_rules! deps {
    () => {
        Bytes!();
        Endian!();
    };
}

macro_rules! LoadCommandData {
    () => {
        deps!();
        # [doc = " The data for a [`macho::LoadCommand`]."] # [derive (Debug , Clone , Copy)] pub struct LoadCommandData < 'data , E : Endian > { cmd : u32 , data : Bytes < 'data > , marker : PhantomData < E > , }
    };
}

LoadCommandData!();