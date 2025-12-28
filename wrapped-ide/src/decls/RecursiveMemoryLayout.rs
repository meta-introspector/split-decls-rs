macro_rules! deps {
    () => {
        MemoryLayoutNode!();
    };
}

macro_rules! RecursiveMemoryLayout {
    () => {
        deps!();
        pub struct RecursiveMemoryLayout { pub nodes : Vec < MemoryLayoutNode > , }
    };
}

RecursiveMemoryLayout!()