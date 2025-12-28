macro_rules! deps {
    () => {
        Decomposition!();
        Composition!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < 'data , I > Composition < 'data , I > where I : Iterator < Item = char > , { fn new (decomposition : Decomposition < 'data , I > , canonical_compositions : Char16Trie < 'data > , composition_passthrough_bound : u16 ,) -> Self { Self { decomposition , canonical_compositions , unprocessed_starter : None , composition_passthrough_bound : u32 :: from (composition_passthrough_bound) , } } # [doc = " Performs canonical composition (including Hangul) on a pair of"] # [doc = " characters or returns `None` if these characters don't compose."] # [doc = " Composition exclusions are taken into account."] # [inline (always)] pub fn compose (& self , starter : char , second : char) -> Option < char > { compose (self . canonical_compositions . iter () , starter , second) } # [doc = " Performs (non-Hangul) canonical composition on a pair of characters"] # [doc = " or returns `None` if these characters don't compose. Composition"] # [doc = " exclusions are taken into account."] # [inline (always)] fn compose_non_hangul (& self , starter : char , second : char) -> Option < char > { compose_non_hangul (self . canonical_compositions . iter () , starter , second) } }
    };
}

impl_57!()