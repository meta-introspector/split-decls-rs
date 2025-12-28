macro_rules! token_governance_impl {
    () => {
        # [decl2 (fn , name = "token_governance_impl" , vis = "pub" , hash = "7ae06f3a")] pub fn token_governance_impl (input : TokenStream) -> TokenStream { let input_str = parse_macro_input ! (input as LitStr) ; let token_amount = input_str . value () ; quote ! { { let tokens : u64 = # token_amount . parse () . unwrap_or (0) ; let (role , voting_power) = match tokens { t if t >= 1000 => ("Senator" , 3) , t if t >= 100 => ("Representative" , 2) , t if t >= 10 => ("Lobbyist" , 1) , _ => ("Observer" , 0) } ; let governance_status = format ! ("TokenHolder {{ tokens: {}, role: '{}', voting_power: {} }}" , tokens , role , voting_power) ; println ! ("cargo:warning=🏛️ Governance role: {} (power: {})" , role , voting_power) ; governance_status } } . into () }
    };
}

token_governance_impl!()