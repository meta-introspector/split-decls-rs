macro_rules! deps {
    () => {
        Easy2Handle!();
        Error!();
    };
}

macro_rules! impl_138 {
    () => {
        deps!();
        impl < H > Easy2Handle < H > { # [doc = " Acquires a reference to the underlying handler for events."] pub fn get_ref (& self) -> & H { self . easy . get_ref () } # [doc = " Acquires a reference to the underlying handler for events."] pub fn get_mut (& mut self) -> & mut H { self . easy . get_mut () } # [doc = " Same as `EasyHandle::set_token`"] pub fn set_token (& mut self , token : usize) -> Result < () , Error > { unsafe { crate :: cvt (curl_sys :: curl_easy_setopt (self . easy . raw () , curl_sys :: CURLOPT_PRIVATE , token ,)) } } impl_easy_getters ! () ; # [doc = " Unpause reading on a connection."] # [doc = ""] # [doc = " Using this function, you can explicitly unpause a connection that was"] # [doc = " previously paused."] # [doc = ""] # [doc = " A connection can be paused by letting the read or the write callbacks"] # [doc = " return `ReadError::Pause` or `WriteError::Pause`."] # [doc = ""] # [doc = " The chance is high that you will get your write callback called before"] # [doc = " this function returns."] pub fn unpause_read (& self) -> Result < () , Error > { self . easy . unpause_read () } # [doc = " Unpause writing on a connection."] # [doc = ""] # [doc = " Using this function, you can explicitly unpause a connection that was"] # [doc = " previously paused."] # [doc = ""] # [doc = " A connection can be paused by letting the read or the write callbacks"] # [doc = " return `ReadError::Pause` or `WriteError::Pause`. A write callback that"] # [doc = " returns pause signals to the library that it couldn't take care of any"] # [doc = " data at all, and that data will then be delivered again to the callback"] # [doc = " when the writing is later unpaused."] pub fn unpause_write (& self) -> Result < () , Error > { self . easy . unpause_write () } # [doc = " Get a pointer to the raw underlying CURL handle."] pub fn raw (& self) -> * mut curl_sys :: CURL { self . easy . raw () } }
    };
}

impl_138!()