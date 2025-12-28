macro_rules! deps {
    () => {
        VswhereInstance!();
    };
}

macro_rules! impl_160 {
    () => {
        deps!();
        impl TryFrom < & Vec < u8 > > for VswhereInstance { type Error = & 'static str ; fn try_from (output : & Vec < u8 >) -> Result < Self , Self :: Error > { let map : HashMap < _ , _ > = output . lines () . map_while (Result :: ok) . filter_map (| s | { let mut splitn = s . splitn (2 , ": ") ; Some ((splitn . next () ? . to_owned () , splitn . next () ? . to_owned ())) }) . collect () ; if ! map . contains_key ("installationName") || ! map . contains_key ("installationPath") || ! map . contains_key ("installationVersion") { return Err ("required properties not found") ; } Ok (Self { map }) } }
    };
}

impl_160!();