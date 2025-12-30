// Generated macro for OpenAIContentPart (enum)
macro_rules! Depcrate_providers_openaiOpenAIContentPart {
() => {
// Module: crate::providers::openai
// Provides: {"OpenAIContentPart"}
// Dependencies: {}
# [doc = " Content part (text or image_url)"] # [derive (Debug , Clone , Serialize , Deserialize)] # [serde (tag = "type")] enum OpenAIContentPart { # [serde (rename = "text")] Text { text : String } , # [serde (rename = "image_url")] ImageUrl { image_url : OpenAIImageUrl } , }
};
}
