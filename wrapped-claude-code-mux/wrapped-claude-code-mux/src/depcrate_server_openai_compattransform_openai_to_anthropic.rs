// Generated macro for transform_openai_to_anthropic (function)
macro_rules! Depcrate_server_openai_compattransform_openai_to_anthropic {
() => {
// Module: crate::server::openai_compat
// Provides: {"transform_openai_to_anthropic"}
// Dependencies: {}
pub fn transform_openai_to_anthropic (openai_request : OpenAIRequest ,) -> Result < AnthropicRequest , String > { let mut anthropic_messages : Vec < Message > = Vec :: new () ; let mut system_prompt : Option < String > = None ; for msg in openai_request . messages { match msg . role . as_str () { "system" => { if system_prompt . is_some () { return Err ("Multiple system messages found in OpenAI request" . to_string ()) ; } system_prompt = Some (msg . content) ; } "user" => anthropic_messages . push (Message { role : "user" . to_string () , content : MessageContent :: Text (msg . content) , }) , "assistant" => anthropic_messages . push (Message { role : "assistant" . to_string () , content : MessageContent :: Text (msg . content) , }) , _ => return Err (format ! ("Unsupported OpenAI message role: {}" , msg . role)) , } } Ok (AnthropicRequest { model : openai_request . model , messages : anthropic_messages , system : system_prompt . map (| s | SystemPrompt :: Text (s)) , stream : Some (openai_request . stream) , max_tokens : 4096 , temperature : None , top_p : None , top_k : None , stop_sequences : None , tools : None , thinking : None , metadata : None , }) }
};
}
