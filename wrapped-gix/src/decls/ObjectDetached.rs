macro_rules! deps {
    () => {
        Kind!();
        Clone!();
    };
}

macro_rules! ObjectDetached {
    () => {
        deps!();
        # [doc = " A detached, self-contained object, without access to its source repository."] # [doc = ""] # [doc = " Use it if an `ObjectRef` should be sent over thread boundaries or stored in collections."] # [derive (Clone)] pub struct ObjectDetached { # [doc = " The id of the object"] pub id : ObjectId , # [doc = " The kind of the object"] pub kind : gix_object :: Kind , # [doc = " The fully decoded object data"] pub data : Vec < u8 > , }
    };
}

ObjectDetached!();