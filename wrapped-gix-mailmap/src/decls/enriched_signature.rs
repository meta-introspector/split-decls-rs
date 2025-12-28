macro_rules! deps {
    () => {
        ResolvedSignature!();
        Signature!();
    };
}

macro_rules! enriched_signature {
    () => {
        deps!();
        fn enriched_signature < 'a > (SignatureRef { name , email , time } : SignatureRef < 'a > , new : ResolvedSignature < '_ > ,) -> Signature < 'a > { match (new . email , new . name) { (Some (new_email) , Some (new_name)) => Signature { email : new_email . to_owned () . into () , name : new_name . to_owned () . into () , time : time . parse () . unwrap_or_default () , } , (Some (new_email) , None) => Signature { email : new_email . to_owned () . into () , name : name . into () , time : time . parse () . unwrap_or_default () , } , (None , Some (new_name)) => Signature { email : email . into () , name : new_name . to_owned () . into () , time : time . parse () . unwrap_or_default () , } , (None , None) => unreachable ! ("BUG: ResolvedSignatures don't exist here when nothing is set") , } }
    };
}

enriched_signature!()