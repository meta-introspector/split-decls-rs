macro_rules! LazyBuffer {
    () => {
        # [derive (Debug , Clone)] pub struct LazyBuffer < I : Iterator > { it : Fuse < I > , buffer : Vec < I :: Item > , }
    };
}

LazyBuffer!()