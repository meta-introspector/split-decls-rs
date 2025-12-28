macro_rules! simple {
    () => {
        # [test] fn simple () { check ! ("" , false , None) ; check ! ("a" , false , None) ; check ! ("peter" , false , None) ; check ! ("Sei gegrüßt, Bärthelt!" , false , None) ; check ! ("أنا لا أتحدث العربية" , false , None) ; check ! ("お前はもう死んでいる" , false , None) ; check ! ("Пушки - интересные музыкальные инструменты" , false , None) ; check ! ("lit 👌 😂 af" , false , None) ; }
    };
}

simple!();