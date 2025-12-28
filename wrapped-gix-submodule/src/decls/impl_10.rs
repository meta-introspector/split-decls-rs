macro_rules! deps {
    () => {
        Branch!();
        Error!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl TryFrom < & BStr > for Branch { type Error = gix_refspec :: parse :: Error ; fn try_from (value : & BStr) -> Result < Self , Self :: Error > { if value == "." { return Ok (Branch :: CurrentInSuperproject) ; } gix_refspec :: parse (value , gix_refspec :: parse :: Operation :: Fetch) . map (| spec | Branch :: Name (spec . source () . expect ("no object") . to_owned ())) } }
    };
}

impl_10!()