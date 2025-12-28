macro_rules! deps {
    () => {
        BorrowSet!();
        LocalizedOutlivesConstraintSet!();
        RegionInferenceContext!();
    };
}

macro_rules! emit_polonius_dump {
    () => {
        deps!();
        # [doc = " The polonius dump consists of:"] # [doc = " - the NLL MIR"] # [doc = " - the list of polonius localized constraints"] # [doc = " - a mermaid graph of the CFG"] # [doc = " - a mermaid graph of the NLL regions and the constraints between them"] # [doc = " - a mermaid graph of the NLL SCCs and the constraints between them"] fn emit_polonius_dump < 'tcx > (dumper : & MirDumper < '_ , '_ , 'tcx > , body : & Body < 'tcx > , regioncx : & RegionInferenceContext < 'tcx > , borrow_set : & BorrowSet < 'tcx > , localized_outlives_constraints : & LocalizedOutlivesConstraintSet , out : & mut dyn io :: Write ,) -> io :: Result < () > { writeln ! (out , "<!DOCTYPE html>") ? ; writeln ! (out , "<html>") ? ; writeln ! (out , "<head><title>Polonius MIR dump</title></head>") ? ; writeln ! (out , "<body>") ? ; writeln ! (out , "<div>") ? ; writeln ! (out , "Raw MIR dump") ? ; writeln ! (out , "<pre><code>") ? ; emit_html_mir (dumper , body , out) ? ; writeln ! (out , "</code></pre>") ? ; writeln ! (out , "</div>") ? ; writeln ! (out , "<div>") ? ; writeln ! (out , "Polonius constraint graph") ? ; writeln ! (out , "<pre class='mermaid'>") ? ; let edge_count = emit_mermaid_constraint_graph (borrow_set , regioncx . liveness_constraints () , & localized_outlives_constraints , out ,) ? ; writeln ! (out , "</pre>") ? ; writeln ! (out , "</div>") ? ; writeln ! (out , "<div>") ? ; writeln ! (out , "Control-flow graph") ? ; writeln ! (out , "<pre class='mermaid'>") ? ; emit_mermaid_cfg (body , out) ? ; writeln ! (out , "</pre>") ? ; writeln ! (out , "</div>") ? ; writeln ! (out , "<div>") ? ; writeln ! (out , "NLL regions") ? ; writeln ! (out , "<pre class='mermaid'>") ? ; emit_mermaid_nll_regions (dumper . tcx () , regioncx , out) ? ; writeln ! (out , "</pre>") ? ; writeln ! (out , "</div>") ? ; writeln ! (out , "<div>") ? ; writeln ! (out , "NLL SCCs") ? ; writeln ! (out , "<pre class='mermaid'>") ? ; emit_mermaid_nll_sccs (dumper . tcx () , regioncx , out) ? ; writeln ! (out , "</pre>") ? ; writeln ! (out , "</div>") ? ; writeln ! (out , "<script src='https://cdn.jsdelivr.net/npm/mermaid/dist/mermaid.min.js'></script>") ? ; writeln ! (out , "<script>") ? ; writeln ! (out , "mermaid.initialize({{ startOnLoad: false, maxEdges: {} }});" , edge_count . max (100) ,) ? ; writeln ! (out , "mermaid.run({{ querySelector: '.mermaid' }})") ? ; writeln ! (out , "</script>") ? ; writeln ! (out , "</body>") ? ; writeln ! (out , "</html>") ? ; Ok (()) }
    };
}

emit_polonius_dump!()