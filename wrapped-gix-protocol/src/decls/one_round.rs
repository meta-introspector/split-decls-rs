macro_rules! deps {
    () => {
        Arguments!();
        Round!();
        Acknowledgement!();
        Error!();
        Response!();
    };
}

macro_rules! one_round {
    () => {
        deps!();
        # [doc = " Prepare to negotiate a single round in the process of letting the remote know what we have, and have in common."] # [doc = ""] # [doc = " Note that this function only configures `arguments`, no IO is performed."] # [doc = ""] # [doc = " The operation is performed with `negotiator` and `graph`, sending the amount of `haves_to_send` after possibly"] # [doc = " making the common commits (as sent by the remote) known to `negotiator` using `previous_response`, if this isn't the first round."] # [doc = " All [commits we have](crate::fetch::Arguments::have()) are added to `arguments` accordingly."] # [doc = ""] # [doc = " Returns information about this round, and `true` if we are done and should stop negotiating *after* the `arguments` have"] # [doc = " been sent to the remote one last time."] pub fn one_round (negotiator : & mut dyn gix_negotiate :: Negotiator , graph : & mut gix_negotiate :: Graph < '_ , '_ > , state : & mut one_round :: State , arguments : & mut crate :: fetch :: Arguments , previous_response : Option < & crate :: fetch :: Response > ,) -> Result < (Round , bool) , Error > { let mut seen_ack = false ; if let Some (response) = previous_response { use crate :: fetch :: response :: Acknowledgement ; for ack in response . acknowledgements () { match ack { Acknowledgement :: Common (id) => { seen_ack = true ; negotiator . in_common_with_remote (* id , graph) ? ; if let Some (common) = & mut state . common_commits { common . push (* id) ; } } Acknowledgement :: Ready => { } Acknowledgement :: Nak => { } } } } if let Some (common) = & mut state . common_commits { for have_id in common { arguments . have (have_id) ; } } let mut haves_added = 0 ; for have_id in (0 .. state . haves_to_send) . map_while (| _ | negotiator . next_have (graph)) { arguments . have (have_id ?) ; haves_added += 1 ; } if seen_ack { state . in_vain = 0 ; } state . seen_ack |= seen_ack ; state . in_vain += haves_added ; let round = Round { haves_sent : haves_added , in_vain : state . in_vain , haves_to_send : state . haves_to_send , previous_response_had_at_least_one_in_common : seen_ack , } ; let is_done = haves_added != state . haves_to_send || (state . seen_ack && state . in_vain >= 256) ; state . adjust_window_size () ; Ok ((round , is_done)) }
    };
}

one_round!();