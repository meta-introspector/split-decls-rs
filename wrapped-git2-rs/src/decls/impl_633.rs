macro_rules! deps {
    () => {
        Direction!();
        Refspec!();
        Error!();
        Buf!();
    };
}

macro_rules! impl_633 {
    () => {
        deps!();
        impl < 'remote > Refspec < 'remote > { # [doc = " Get the refspec's direction."] pub fn direction (& self) -> Direction { match unsafe { raw :: git_refspec_direction (self . raw) } { raw :: GIT_DIRECTION_FETCH => Direction :: Fetch , raw :: GIT_DIRECTION_PUSH => Direction :: Push , n => panic ! ("unknown refspec direction: {}" , n) , } } # [doc = " Get the destination specifier."] # [doc = ""] # [doc = " If the destination is not utf-8, None is returned."] pub fn dst (& self) -> Option < & str > { str :: from_utf8 (self . dst_bytes ()) . ok () } # [doc = " Get the destination specifier, in bytes."] pub fn dst_bytes (& self) -> & [u8] { unsafe { crate :: opt_bytes (self , raw :: git_refspec_dst (self . raw)) . unwrap () } } # [doc = " Check if a refspec's destination descriptor matches a reference"] pub fn dst_matches (& self , refname : & str) -> bool { let refname = CString :: new (refname) . unwrap () ; unsafe { raw :: git_refspec_dst_matches (self . raw , refname . as_ptr ()) == 1 } } # [doc = " Get the source specifier."] # [doc = ""] # [doc = " If the source is not utf-8, None is returned."] pub fn src (& self) -> Option < & str > { str :: from_utf8 (self . src_bytes ()) . ok () } # [doc = " Get the source specifier, in bytes."] pub fn src_bytes (& self) -> & [u8] { unsafe { crate :: opt_bytes (self , raw :: git_refspec_src (self . raw)) . unwrap () } } # [doc = " Check if a refspec's source descriptor matches a reference"] pub fn src_matches (& self , refname : & str) -> bool { let refname = CString :: new (refname) . unwrap () ; unsafe { raw :: git_refspec_src_matches (self . raw , refname . as_ptr ()) == 1 } } # [doc = " Get the force update setting."] pub fn is_force (& self) -> bool { unsafe { raw :: git_refspec_force (self . raw) == 1 } } # [doc = " Get the refspec's string."] # [doc = ""] # [doc = " Returns None if the string is not valid utf8."] pub fn str (& self) -> Option < & str > { str :: from_utf8 (self . bytes ()) . ok () } # [doc = " Get the refspec's string as a byte array"] pub fn bytes (& self) -> & [u8] { unsafe { crate :: opt_bytes (self , raw :: git_refspec_string (self . raw)) . unwrap () } } # [doc = " Transform a reference to its target following the refspec's rules"] pub fn transform (& self , name : & str) -> Result < Buf , Error > { let name = CString :: new (name) . unwrap () ; unsafe { let buf = Buf :: new () ; try_call ! (raw :: git_refspec_transform (buf . raw () , self . raw , name . as_ptr ())) ; Ok (buf) } } # [doc = " Transform a target reference to its source reference following the refspec's rules"] pub fn rtransform (& self , name : & str) -> Result < Buf , Error > { let name = CString :: new (name) . unwrap () ; unsafe { let buf = Buf :: new () ; try_call ! (raw :: git_refspec_rtransform (buf . raw () , self . raw , name . as_ptr ())) ; Ok (buf) } } }
    };
}

impl_633!()