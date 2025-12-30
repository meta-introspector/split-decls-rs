// Generated macro for impl_70 (impl)
macro_rules! Depcrate_listingimpl_70 {
() => {
// Module: crate::listing
// Provides: {"impl_70"}
// Dependencies: {}
impl < 'e > RewriteState < 'e > { fn open_listing (& mut self , tag : pulldown_cmark :: CowStr < '_ > , mode : Mode ,) -> Result < () , String > { let listing = ListingBuilder :: from_tag (& tag) ? . build (mode) ; let opening_event = Event :: Html (listing . opening_html () . into ()) ; self . current = Some (listing) ; self . events . push (Ok (opening_event)) ; Ok (()) } fn close_listing (& mut self , tag : pulldown_cmark :: CowStr < '_ >) { let trailing = if ! tag . ends_with ('>') { tag . replace ("</Listing>" , "") } else { String :: from ("") } ; match & self . current { Some (listing) => { let closing_event = Event :: Html (listing . closing_html (& trailing) . into ()) ; self . current = None ; self . events . push (Ok (closing_event)) ; } None => { self . events . push (Err (String :: from ("Closing `</Listing>` without opening tag." ,))) ; } } } }
};
}
