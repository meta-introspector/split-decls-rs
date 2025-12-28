macro_rules! deps {
    () => {
        Commit!();
        ObjectDetached!();
        Repository!();
        Kind!();
    };
}

macro_rules! impl_158 {
    () => {
        deps!();
        # [doc = " Remove Lifetime"] impl Commit < '_ > { # [doc = " Create an owned instance of this object, copying our data in the process."] pub fn detached (& self) -> ObjectDetached { ObjectDetached { id : self . id , kind : gix_object :: Kind :: Commit , data : self . data . clone () , } } # [doc = " Sever the connection to the `Repository` and turn this instance into a standalone object."] pub fn detach (self) -> ObjectDetached { self . into () } # [doc = " Retrieve this instance's encoded data, leaving its own data empty."] # [doc = ""] # [doc = " This method works around the immovability of members of this type."] pub fn take_data (& mut self) -> Vec < u8 > { std :: mem :: take (& mut self . data) } }
    };
}

impl_158!()