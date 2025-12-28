macro_rules! deps {
    () => {
        Graph!();
        Error!();
    };
}

macro_rules! Negotiator {
    () => {
        deps!();
        # [doc = " A delegate to implement a negotiation algorithm."] pub trait Negotiator { # [doc = " Mark `id` as common between the remote and us."] # [doc = ""] # [doc = " These ids are typically the local tips of remote tracking branches."] fn known_common (& mut self , id : gix_hash :: ObjectId , graph : & mut Graph < '_ , '_ >) -> Result < () , Error > ; # [doc = " Add `id` as starting point of a traversal across commits that aren't necessarily common between the remote and us."] # [doc = ""] # [doc = " These tips are usually the commits of local references whose tips should lead to objects that we have in common with the remote."] fn add_tip (& mut self , id : gix_hash :: ObjectId , graph : & mut Graph < '_ , '_ >) -> Result < () , Error > ; # [doc = " Produce the next id of an object that we want the server to know we have. It's an object we don't know we have in common or not."] # [doc = ""] # [doc = " Returns `None` if we have exhausted all options, which might mean we have traversed the entire commit graph."] fn next_have (& mut self , graph : & mut Graph < '_ , '_ >) -> Option < Result < gix_hash :: ObjectId , Error > > ; # [doc = " Mark `id` as being common with the remote (as informed by the remote itself) and return `true` if we knew it was common already."] # [doc = ""] # [doc = " We can assume to have already seen `id` as we were the one to inform the remote in a prior `have`."] fn in_common_with_remote (& mut self , id : gix_hash :: ObjectId , graph : & mut Graph < '_ , '_ >) -> Result < bool , Error > ; }
    };
}

Negotiator!()