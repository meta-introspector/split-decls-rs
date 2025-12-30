// Generated macro for Emoji (struct)
macro_rules! Depcrate_utilsEmoji {
() => {
// Module: crate::utils
// Provides: {"Emoji"}
// Dependencies: {}
# [doc = " \"Intelligent\" emoji formatter."] # [doc = ""] # [doc = " This struct intelligently wraps an emoji so that it is rendered"] # [doc = " only on systems that want emojis and renders a fallback on others."] # [doc = ""] # [doc = " Example:"] # [doc = ""] # [doc = " ```rust"] # [doc = " use console::Emoji;"] # [doc = " println!(\"[3/4] {}Downloading ...\", Emoji(\"🚚 \", \"\"));"] # [doc = " println!(\"[4/4] {} Done!\", Emoji(\"✨\", \":-)\"));"] # [doc = " ```"] # [derive (Copy , Clone)] pub struct Emoji < 'a , 'b > (pub & 'a str , pub & 'b str) ;
};
}
