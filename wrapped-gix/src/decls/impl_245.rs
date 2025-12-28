macro_rules! deps {
    () => {
        Object!();
        Repository!();
        ObjectDetached!();
    };
}

macro_rules! impl_245 {
    () => {
        deps!();
        impl Object < '_ > { # [doc = " Create an owned instance of this object, copying our data in the process."] pub fn detached (& self) -> ObjectDetached { ObjectDetached { id : self . id , kind : self . kind , data : self . data . clone () , } } # [doc = " Sever the connection to the `Repository` and turn this instance into a standalone object."] pub fn detach (self) -> ObjectDetached { self . into () } }
    };
}

impl_245!()