// Generated macro for SPLIT_TEST_ITEMS (static)
macro_rules! DepcrateSPLIT_TEST_ITEMS {
() => {
// Module: crate
// Provides: {"SPLIT_TEST_ITEMS"}
// Dependencies: {}
# [cfg (test)] static SPLIT_TEST_ITEMS : & 'static [(& 'static str , Option < & 'static [& 'static str] >)] = & [("foo$baz" , Some (& ["foo$baz"])) , ("foo baz" , Some (& ["foo" , "baz"])) , ("foo\"bar\"baz" , Some (& ["foobarbaz"])) , ("foo \"bar\"baz" , Some (& ["foo" , "barbaz"])) , ("   foo \nbar" , Some (& ["foo" , "bar"])) , ("foo\\\nbar" , Some (& ["foobar"])) , ("\"foo\\\nbar\"" , Some (& ["foobar"])) , ("'baz\\$b'" , Some (& ["baz\\$b"])) , ("'baz\\\''" , None) , ("\\" , None) , ("\"\\" , None) , ("'\\" , None) , ("\"" , None) , ("'" , None) , ("foo #bar\nbaz" , Some (& ["foo" , "baz"])) , ("foo #bar" , Some (& ["foo"])) , ("foo#bar" , Some (& ["foo#bar"])) , ("foo\"#bar" , None) , ("'\\n'" , Some (& ["\\n"])) , ("'\\\\n'" , Some (& ["\\\\n"])) ,] ;
};
}
