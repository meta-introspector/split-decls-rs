// Generated macro for color_tag (function)
macro_rules! Depcrate_parse_color_tagcolor_tag {
() => {
// Module: crate::parse::color_tag
// Provides: {"color_tag"}
// Dependencies: {}
# [doc = " Parses a color tag."] pub fn color_tag (input : Input < '_ >) -> Result < '_ , ColorTag > { let tag = alt ((map (tuple ((tag ("</") , space0 , tag (">"))) , | _ | (true , vec ! [])) , delimited (tag ("<") , alt ((map (preceded (tag ("/") , spaced (separated_list1 (stag (",") , spaced (attr)))) , | attrs | (true , attrs)) , map (separated_list1 (stag (",") , spaced (attr)) , | attrs | (false , attrs)) ,)) , tag (">") ,) ,)) ; with_failure_message (map (consumed (tag) , | (source , (is_close , changes)) | ColorTag { source : Some (source) , span : None , is_close , change_set : ChangeSet :: from (changes . as_ref ()) , }) , "Unable to parse this tag") (input) }
};
}
