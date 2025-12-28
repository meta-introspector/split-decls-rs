macro_rules! Addr {
    () => {
        # [doc = " An address within a data stream. Each data stream has its own address space,"] # [doc = " i.e. the first piece of data written to the events stream will have"] # [doc = " `Addr(0)` and the first piece of data written to the string data stream"] # [doc = " will *also* have `Addr(0)`."] # [derive (Clone , Copy , Eq , PartialEq , Debug)] pub struct Addr (pub u64) ;
    };
}

Addr!()