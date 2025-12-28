macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! MemoryUsage {
    () => {
        deps!();
        # [derive (Copy , Clone)] pub struct MemoryUsage { pub allocated : Bytes , }
    };
}

MemoryUsage!();