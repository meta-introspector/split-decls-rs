macro_rules! SubstitutionHighlight {
    () => {
        # [doc = " Used to translate between `Span`s and byte positions within a single output line in highlighted"] # [doc = " code of structured suggestions."] # [derive (Debug , Clone , Copy)] pub (crate) struct SubstitutionHighlight { pub (crate) start : usize , pub (crate) end : usize , }
    };
}

SubstitutionHighlight!()