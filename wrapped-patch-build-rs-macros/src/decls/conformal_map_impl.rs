macro_rules! conformal_map_impl {
    () => {
        # [decl2 (fn , name = "conformal_map_impl" , vis = "pub" , hash = "3d364cc6")] pub fn conformal_map_impl (input : TokenStream) -> TokenStream { let input_str = parse_macro_input ! (input as LitStr) ; let rust_graph = input_str . value () ; quote ! { { println ! ("cargo:warning=🌀 Computing conformal mapping") ; let node_count = # rust_graph . matches ("rustc_") . count () ; let edge_count = # rust_graph . matches ("->") . count () ; let euler_char = node_count as i32 - edge_count as i32 ; let genus = (2 - euler_char) / 2 ; let math_object = match (node_count , genus) { (n , g) if n > 100 && g < 0 => "Monster Group M" , (n , _) if n > 50 => "Leech Lattice Λ₂₄" , (n , _) if n > 20 => "E₈ Exceptional Group" , _ => "Finite Simple Group" } ; let mapping = format ! ("ConformalMap {{ rustc_nodes: {}, edges: {}, genus: {}, target: '{}' }}" , node_count , edge_count , genus , math_object) ; println ! ("cargo:warning=🎯 Mapped to: {}" , math_object) ; mapping } } . into () }
    };
}

conformal_map_impl!();