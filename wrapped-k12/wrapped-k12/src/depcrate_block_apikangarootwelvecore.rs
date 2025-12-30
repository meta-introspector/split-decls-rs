// Generated macro for KangarooTwelveCore (struct)
macro_rules! Depcrate_block_apiKangarooTwelveCore {
() => {
// Module: crate::block_api
// Provides: {"KangarooTwelveCore"}
// Dependencies: {}
# [doc = " Core [`KangarooTwelve`] hasher state."] # [derive (Clone)] # [allow (non_camel_case_types)] pub struct KangarooTwelveCore < 'cs > { customization : & 'cs [u8] , buffer : [u8 ; CHUNK_SIZE] , bufpos : usize , final_tshk : TurboShake128 < 0x06 > , chain_tshk : TurboShake128 < 0x0B > , chain_length : usize , }
};
}
