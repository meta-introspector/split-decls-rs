macro_rules! deps {
    () => {
        Seed!();
    };
}

macro_rules! SEEDS {
    () => {
        deps!();
        const SEEDS : & 'static [Seed] = & [Seed :: new ("" , "" , Some (0) , Some (0)) , Seed :: new ("" , "a" , Some (0) , Some (1)) , Seed :: new ("" , "ab" , Some (0) , Some (2)) , Seed :: new ("" , "abc" , Some (0) , Some (3)) , Seed :: new ("a" , "" , None , None) , Seed :: new ("a" , "a" , Some (0) , Some (0)) , Seed :: new ("a" , "aa" , Some (0) , Some (1)) , Seed :: new ("a" , "ba" , Some (1) , Some (1)) , Seed :: new ("a" , "bba" , Some (2) , Some (2)) , Seed :: new ("a" , "bbba" , Some (3) , Some (3)) , Seed :: new ("a" , "bbbab" , Some (3) , Some (3)) , Seed :: new ("a" , "bbbabb" , Some (3) , Some (3)) , Seed :: new ("a" , "bbbabbb" , Some (3) , Some (3)) , Seed :: new ("a" , "bbbbbb" , None , None) , Seed :: new ("ab" , "" , None , None) , Seed :: new ("ab" , "a" , None , None) , Seed :: new ("ab" , "b" , None , None) , Seed :: new ("ab" , "ab" , Some (0) , Some (0)) , Seed :: new ("ab" , "aab" , Some (1) , Some (1)) , Seed :: new ("ab" , "aaab" , Some (2) , Some (2)) , Seed :: new ("ab" , "abaab" , Some (0) , Some (3)) , Seed :: new ("ab" , "baaab" , Some (3) , Some (3)) , Seed :: new ("ab" , "acb" , None , None) , Seed :: new ("ab" , "abba" , Some (0) , Some (0)) , Seed :: new ("abc" , "ab" , None , None) , Seed :: new ("abc" , "abc" , Some (0) , Some (0)) , Seed :: new ("abc" , "abcz" , Some (0) , Some (0)) , Seed :: new ("abc" , "abczz" , Some (0) , Some (0)) , Seed :: new ("abc" , "zabc" , Some (1) , Some (1)) , Seed :: new ("abc" , "zzabc" , Some (2) , Some (2)) , Seed :: new ("abc" , "azbc" , None , None) , Seed :: new ("abc" , "abzc" , None , None) , Seed :: new ("abczdef" , "abczdefzzzzzzzzzzzzzzzzzzzz" , Some (0) , Some (0)) , Seed :: new ("abczdef" , "zzzzzzzzzzzzzzzzzzzzabczdef" , Some (20) , Some (20)) , Seed :: new ("xyz" , "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaxyz" , Some (32) , Some (32) ,) , Seed :: new ("\u{0}\u{15}" , "\u{0}\u{15}\u{15}\u{0}" , Some (0) , Some (0)) , Seed :: new ("\u{0}\u{1e}" , "\u{1e}\u{0}" , None , None) ,] ;
    };
}

SEEDS!();