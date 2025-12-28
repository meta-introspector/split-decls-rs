macro_rules! HmacImpl {
    () => {
        # [doc = " Trait representing a HMAC implementation."] # [doc = ""] # [doc = " Most users should use [`Hmac`] or [`SimpleHmac`]."] pub trait HmacImpl : Clone + OutputSizeUser { # [doc = " Create new HMAC state with the given key."] fn new_from_slice (key : & [u8]) -> Self ; # [doc = " Update HMAC state."] fn update (& mut self , data : & [u8]) ; # [doc = " Finalize the HMAC state and get generated tag."] fn finalize (self) -> Output < Self > ; }
    };
}

HmacImpl!()