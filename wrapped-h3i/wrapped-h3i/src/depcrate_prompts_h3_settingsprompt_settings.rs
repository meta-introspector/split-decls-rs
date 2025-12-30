// Generated macro for prompt_settings (function)
macro_rules! Depcrate_prompts_h3_settingsprompt_settings {
() => {
// Module: crate::prompts::h3::settings
// Provides: {"prompt_settings"}
// Dependencies: {}
pub fn prompt_settings () -> InquireResult < Action > { let stream_id = h3 :: prompt_control_stream_id () ? ; let settings = settings_read_loop () ; let fin_stream = prompt_fin_stream () ? ; let action = Action :: SendFrame { stream_id , fin_stream , frame : quiche :: h3 :: frame :: Frame :: Settings { max_field_section_size : None , qpack_max_table_capacity : None , qpack_blocked_streams : None , connect_protocol_enabled : None , h3_datagram : None , grease : None , raw : None , additional_settings : Some (settings) , } , } ; Ok (action) }
};
}
