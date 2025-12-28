macro_rules! deps {
    () => {
        VersionId!();
        Version!();
        Versions!();
        VersionData!();
    };
}

macro_rules! impl_1167 {
    () => {
        deps!();
        impl < 'data > Versions < 'data > { # [doc = " Add a version."] pub fn add (& mut self , data : VersionData < 'data >) -> VersionId { let id = self . next_id () ; self . push (Version { id , data , delete : false , }) ; id } }
    };
}

impl_1167!()