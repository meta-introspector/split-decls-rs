// Generated macro for returns_summarizable (function)
macro_rules! Depcratereturns_summarizable {
() => {
// Module: crate
// Provides: {"returns_summarizable"}
// Dependencies: {}
fn returns_summarizable (switch : bool) -> impl Summary { if switch { NewsArticle { headline : String :: from ("Penguins win the Stanley Cup Championship!" ,) , location : String :: from ("Pittsburgh, PA, USA") , author : String :: from ("Iceburgh") , content : String :: from ("The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL." ,) , } } else { SocialPost { username : String :: from ("horse_ebooks") , content : String :: from ("of course, as you probably already know, people" ,) , reply : false , repost : false , } } }
};
}
