macro_rules! deps {
    () => {
        Error!();
        Message!();
        Easy2Handle!();
        EasyHandle!();
    };
}

macro_rules! impl_142 {
    () => {
        deps!();
        impl < 'multi > Message < 'multi > { # [doc = " If this message indicates that a transfer has finished, returns the"] # [doc = " result of the transfer in `Some`."] # [doc = ""] # [doc = " If the message doesn't indicate that a transfer has finished, then"] # [doc = " `None` is returned."] # [doc = ""] # [doc = " Note that the `result*_for` methods below should be preferred as they"] # [doc = " provide better error messages as the associated error data on the"] # [doc = " handle can be associated with the error type."] pub fn result (& self) -> Option < Result < () , Error > > { unsafe { if (* self . ptr) . msg == curl_sys :: CURLMSG_DONE { Some (crate :: cvt ((* self . ptr) . data as curl_sys :: CURLcode)) } else { None } } } # [doc = " Same as `result`, except only returns `Some` for the specified handle."] # [doc = ""] # [doc = " Note that this function produces better error messages than `result` as"] # [doc = " it uses `take_error_buf` to associate error information with the"] # [doc = " returned error."] pub fn result_for (& self , handle : & EasyHandle) -> Option < Result < () , Error > > { if ! self . is_for (handle) { return None ; } let mut err = self . result () ; if let Some (Err (e)) = & mut err { if let Some (s) = handle . easy . take_error_buf () { e . set_extra (s) ; } } err } # [doc = " Same as `result`, except only returns `Some` for the specified handle."] # [doc = ""] # [doc = " Note that this function produces better error messages than `result` as"] # [doc = " it uses `take_error_buf` to associate error information with the"] # [doc = " returned error."] pub fn result_for2 < H > (& self , handle : & Easy2Handle < H >) -> Option < Result < () , Error > > { if ! self . is_for2 (handle) { return None ; } let mut err = self . result () ; if let Some (Err (e)) = & mut err { if let Some (s) = handle . easy . take_error_buf () { e . set_extra (s) ; } } err } # [doc = " Returns whether this easy message was for the specified easy handle or"] # [doc = " not."] pub fn is_for (& self , handle : & EasyHandle) -> bool { unsafe { (* self . ptr) . easy_handle == handle . easy . raw () } } # [doc = " Same as `is_for`, but for `Easy2Handle`."] pub fn is_for2 < H > (& self , handle : & Easy2Handle < H >) -> bool { unsafe { (* self . ptr) . easy_handle == handle . easy . raw () } } # [doc = " Returns the token associated with the easy handle that this message"] # [doc = " represents a completion for."] # [doc = ""] # [doc = " This function will return the token assigned with"] # [doc = " `EasyHandle::set_token`. This reads the `CURLINFO_PRIVATE` field of the"] # [doc = " underlying `*mut CURL`."] pub fn token (& self) -> Result < usize , Error > { unsafe { let mut p = 0usize ; crate :: cvt (curl_sys :: curl_easy_getinfo ((* self . ptr) . easy_handle , curl_sys :: CURLINFO_PRIVATE , & mut p ,)) ? ; Ok (p) } } }
    };
}

impl_142!()