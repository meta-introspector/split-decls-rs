// Generated macro for prompt_priority (function)
macro_rules! Depcrate_prompts_h3_priorityprompt_priority {
() => {
// Module: crate::prompts::h3::priority
// Provides: {"prompt_priority"}
// Dependencies: {}
pub fn prompt_priority () -> InquireResult < Action > { let stream_id = h3 :: prompt_stream_id () ? ; let ty = prompt_request_or_push () ? ; let prioritized_element_id = h3 :: prompt_varint ("Prioritized Element ID:") ? ; let priority_field_value = Text :: new ("priority field value:") . prompt () ? ; let frame = if ty . as_str () == REQUEST { quiche :: h3 :: frame :: Frame :: PriorityUpdateRequest { prioritized_element_id , priority_field_value : priority_field_value . into () , } } else { quiche :: h3 :: frame :: Frame :: PriorityUpdatePush { prioritized_element_id , priority_field_value : priority_field_value . into () , } } ; let fin_stream = prompt_fin_stream () ? ; let action = Action :: SendFrame { stream_id , fin_stream , frame , } ; Ok (action) }
};
}
