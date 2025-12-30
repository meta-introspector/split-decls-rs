// Generated macro for by_name_or_url (function)
macro_rules! Depcrate_repository_remoteby_name_or_url {
() => {
// Module: crate::repository::remote
// Provides: {"by_name_or_url"}
// Dependencies: {}
# [cfg (any (feature = "blocking-client" , feature = "async-client"))] pub (crate) fn by_name_or_url < 'repo > (repo : & 'repo gix :: Repository , name_or_url : Option < & str > ,) -> anyhow :: Result < gix :: Remote < 'repo > > { repo . find_fetch_remote (name_or_url . map (Into :: into)) . map_err (Into :: into) }
};
}
