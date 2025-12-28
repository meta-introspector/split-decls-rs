macro_rules! BY_NAME {
    () => {
        pub const BY_NAME : & 'static [(& 'static str , & 'static [(char , char)])] = & [("ALetter" , ALETTER) , ("CR" , CR) , ("Double_Quote" , DOUBLE_QUOTE) , ("Extend" , EXTEND) , ("ExtendNumLet" , EXTENDNUMLET) , ("Format" , FORMAT) , ("Hebrew_Letter" , HEBREW_LETTER) , ("Katakana" , KATAKANA) , ("LF" , LF) , ("MidLetter" , MIDLETTER) , ("MidNum" , MIDNUM) , ("MidNumLet" , MIDNUMLET) , ("Newline" , NEWLINE) , ("Numeric" , NUMERIC) , ("Regional_Indicator" , REGIONAL_INDICATOR) , ("Single_Quote" , SINGLE_QUOTE) , ("WSegSpace" , WSEGSPACE) , ("ZWJ" , ZWJ) ,] ;
    };
}

BY_NAME!();