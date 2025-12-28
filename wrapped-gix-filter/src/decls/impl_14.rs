macro_rules! deps {
    () => {
        AttributesDigest!();
        Configuration!();
        Mode!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl AttributesDigest { # [doc = " Return the end-of-line mode this digest would require, or `None` if no conversion would be performed."] pub fn to_eol (& self , config : Configuration) -> Option < Mode > { Some (match self { AttributesDigest :: Binary => return None , AttributesDigest :: TextInput | AttributesDigest :: TextAutoInput => Mode :: Lf , AttributesDigest :: TextCrlf | AttributesDigest :: TextAutoCrlf => Mode :: CrLf , AttributesDigest :: Text | AttributesDigest :: TextAuto => config . to_eol () , }) } # [doc = " Return true if this digest allows for auto-determination of CRLF text conversion."] pub fn is_auto_text (& self) -> bool { matches ! (self , AttributesDigest :: TextAuto | AttributesDigest :: TextAutoCrlf | AttributesDigest :: TextAutoInput) } }
    };
}

impl_14!();