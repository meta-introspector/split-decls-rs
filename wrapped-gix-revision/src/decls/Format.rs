macro_rules! Format {
    () => {
        # [doc = " A structure implementing `Display`, producing a `git describe` like string."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] pub struct Format < 'a > { # [doc = " The name of the branch or tag to display, as is."] # [doc = ""] # [doc = " If `None`, the `id` will be displayed as a fallback."] pub name : Option < Cow < 'a , BStr > > , # [doc = " The `id` of the commit to describe."] pub id : gix_hash :: ObjectId , # [doc = " The amount of hex characters to use to display `id`."] pub hex_len : usize , # [doc = " The amount of commits between `name` and `id`, where `id` is in the future of `name`."] pub depth : u32 , # [doc = " If true, the long form of the describe string will be produced even if `id` lies directly on `name`,"] # [doc = " hence has a depth of 0."] pub long : bool , # [doc = " If `Some(suffix)`, it will be appended to the describe string."] # [doc = " This should be set if the working tree was determined to be dirty."] pub dirty_suffix : Option < String > , }
    };
}

Format!();