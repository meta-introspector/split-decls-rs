// Generated macro for transform_anthropic_to_openai (function)
macro_rules! Depcrate_server_openai_compattransform_anthropic_to_openai {
() => {
// Module: crate::server::openai_compat
// Provides: {"transform_anthropic_to_openai"}
// Dependencies: {}
pub fn transform_anthropic_to_openai (anthropic_response : AnthropicResponse , original_model : String ,) -> OpenAIResponse { let choices = anthropic_response . content . into_iter () . filter_map (| msg_content | { if let MessageContent :: Text (text) = msg_content { Some (OpenAIChoice { index : 0 , message : OpenAIMessage { role : "assistant" . to_string () , content : text , } , finish_reason : anthropic_response . stop_reason . unwrap_or ("stop" . to_string ()) , }) } else { warn ! ("Anthropic response contained non-text content, skipping for OpenAI conversion.") ; None } }) . collect () ; OpenAIResponse { id : anthropic_response . id , object : "chat.completion" . to_string () , created : anthropic_response . stop_sequence . map_or (0 , | _ | { std :: time :: SystemTime :: now () . duration_since (std :: time :: UNIX_EPOCH) . unwrap_or_default () . as_secs () }) , model : original_model , choices , usage : OpenAIUsage { prompt_tokens : anthropic_response . usage . as_ref () . map_or (0 , | u | u . input_tokens) , completion_tokens : anthropic_response . usage . as_ref () . map_or (0 , | u | u . output_tokens) , total_tokens : anthropic_response . usage . as_ref () . map_or (0 , | u | u . input_tokens + u . output_tokens) , } , } }
};
}
