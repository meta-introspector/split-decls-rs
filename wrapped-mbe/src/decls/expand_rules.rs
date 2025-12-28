macro_rules! deps {
    () => {
        MatchedArmIndex!();
        ExpandResult!();
        Rule!();
        ExpandError!();
        ExpandErrorKind!();
    };
}

macro_rules! expand_rules {
    () => {
        deps!();
        pub (crate) fn expand_rules (rules : & [crate :: Rule] , input : & tt :: TopSubtree < Span > , marker : impl Fn (& mut Span) + Copy , call_site : Span , def_site_edition : Edition ,) -> ExpandResult < (tt :: TopSubtree < Span > , MatchedArmIndex) > { let mut match_ : Option < (matcher :: Match < '_ > , & crate :: Rule , usize) > = None ; for (idx , rule) in rules . iter () . enumerate () { let new_match = matcher :: match_ (& rule . lhs , input , def_site_edition) ; if new_match . err . is_none () { let ExpandResult { value , err : transcribe_err } = transcriber :: transcribe (& rule . rhs , & new_match . bindings , marker , call_site) ; if transcribe_err . is_none () { return ExpandResult :: ok ((value , Some (idx as u32))) ; } } if let Some ((prev_match , _ , _)) = & match_ { if (new_match . unmatched_tts , - (new_match . bound_count as i32)) < (prev_match . unmatched_tts , - (prev_match . bound_count as i32)) { match_ = Some ((new_match , rule , idx)) ; } } else { match_ = Some ((new_match , rule , idx)) ; } } if let Some ((match_ , rule , idx)) = match_ { let ExpandResult { value , err : transcribe_err } = transcriber :: transcribe (& rule . rhs , & match_ . bindings , marker , call_site) ; ExpandResult { value : (value , idx . try_into () . ok ()) , err : match_ . err . or (transcribe_err) } } else { ExpandResult :: new ((tt :: TopSubtree :: empty (tt :: DelimSpan :: from_single (call_site)) , None) , ExpandError :: new (call_site , ExpandErrorKind :: NoMatchingRule) ,) } }
    };
}

expand_rules!()