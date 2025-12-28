macro_rules! deps {
    () => {
        SearchTest!();
    };
}

macro_rules! TEDDY {
    () => {
        deps!();
        const TEDDY : & 'static [SearchTest] = & [t ! (teddy010 , & ["a" , "b" , "c" , "d" , "e" , "f" , "g" , "h" , "i" , "j" , "k"] , "abcdefghijk" , & [(0 , 0 , 1) , (1 , 1 , 2) , (2 , 2 , 3) , (3 , 3 , 4) , (4 , 4 , 5) , (5 , 5 , 6) , (6 , 6 , 7) , (7 , 7 , 8) , (8 , 8 , 9) , (9 , 9 , 10) , (10 , 10 , 11)]) , t ! (teddy020 , & ["ab" , "bc" , "cd" , "de" , "ef" , "fg" , "gh" , "hi" , "ij" , "jk" , "kl"] , "abcdefghijk" , & [(0 , 0 , 2) , (2 , 2 , 4) , (4 , 4 , 6) , (6 , 6 , 8) , (8 , 8 , 10) ,]) , t ! (teddy030 , & ["abc"] , "abcdefghijklmnopqrstuvwxyzabcdefghijk" , & [(0 , 0 , 3) , (0 , 26 , 29)]) ,] ;
    };
}

TEDDY!()