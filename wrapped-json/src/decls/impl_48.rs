macro_rules! deps {
    () => {
        StreamDeserializer!();
        Result!();
        Read!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < 'de , R , T > Iterator for StreamDeserializer < 'de , R , T > where R : Read < 'de > , T : de :: Deserialize < 'de > , { type Item = Result < T > ; fn next (& mut self) -> Option < Result < T > > { if R :: should_early_return_if_failed && self . failed { return None ; } match self . de . parse_whitespace () { Ok (None) => { self . offset = self . de . read . byte_offset () ; None } Ok (Some (b)) => { let self_delineated_value = match b { b'[' | b'"' | b'{' => true , _ => false , } ; self . offset = self . de . read . byte_offset () ; let result = de :: Deserialize :: deserialize (& mut self . de) ; Some (match result { Ok (value) => { self . offset = self . de . read . byte_offset () ; if self_delineated_value { Ok (value) } else { self . peek_end_of_value () . map (| () | value) } } Err (e) => { self . de . read . set_failed (& mut self . failed) ; Err (e) } }) } Err (e) => { self . de . read . set_failed (& mut self . failed) ; Some (Err (e)) } } } }
    };
}

impl_48!()