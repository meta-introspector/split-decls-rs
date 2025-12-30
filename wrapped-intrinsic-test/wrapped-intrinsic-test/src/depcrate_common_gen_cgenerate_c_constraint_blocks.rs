// Generated macro for generate_c_constraint_blocks (function)
macro_rules! Depcrate_common_gen_cgenerate_c_constraint_blocks {
() => {
// Module: crate::common::gen_c
// Provides: {"generate_c_constraint_blocks"}
// Dependencies: {}
pub fn generate_c_constraint_blocks < 'a , T : IntrinsicTypeDefinition + 'a > (w : & mut impl std :: io :: Write , intrinsic : & Intrinsic < T > , indentation : Indentation , constraints : & mut (impl Iterator < Item = & 'a Argument < T > > + Clone) , name : String ,) -> std :: io :: Result < () > { let Some (current) = constraints . next () else { return generate_c_test_loop (w , intrinsic , indentation , & name , PASSES) ; } ; let body_indentation = indentation . nested () ; for i in current . constraint . iter () . flat_map (| c | c . iter ()) { let ty = current . ty . c_type () ; writeln ! (w , "{indentation}{{") ? ; writeln ! (w , "{body_indentation}const {ty} {} = ({ty}){i};" , current . generate_name ()) ? ; generate_c_constraint_blocks (w , intrinsic , body_indentation , & mut constraints . clone () , format ! ("{name}-{i}") ,) ? ; writeln ! (w , "{indentation}}}") ? ; } Ok (()) }
};
}
