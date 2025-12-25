# TOP 3 N-GRAMS PER SIZE - LAYER 0 ONLY
========================================

## LAYER 0 (17,377 → 13,901 tokens)

### Top 3 2-Grams:
1. **output2 → output2** (count: 14,004, type: generic_pair)
2. **output2 → wrapped** (count: 13,999, type: wrapped_module)
3. **src → decls** (count: 11,715, type: source_declaration)

### Top 3 3-Grams:
1. **output2 → output2 → wrapped** (count: 13,999, type: output_wrapped_path)
2. **src → decls → wrapped** (count: 11,076, type: source_declaration_path)
3. **output2 → git → objects** (count: 3,274, type: generic_triple)

### Top 3 5-Grams:
1. **decls → decl → module → invocation → rs** (count: 586, type: generic_quintuple)
2. **src → decls → decl → module → invocation** (count: 586, type: complete_module_path)
3. **wrapped → patch → build → rs → macros** (count: 331, type: generic_quintuple)

### Top 3 7-Grams:
1. **output2 → wrapped → hir → src → decls → wrapped → hir** (count: 194, type: generic_septuple)
2. **wrapped → hir → src → decls → wrapped → hir → decls** (count: 194, type: generic_septuple)
3. **output2 → output2 → wrapped → hir → src → decls → wrapped** (count: 194, type: generic_septuple)

---

## LAYERS 1-7: EMOJI COMPRESSION
All subsequent layers (1-7) contain emoji tokens instead of file paths, 
so traditional n-gram analysis doesn't apply. The compression transforms:
- Layer 1-7: File paths → Emoji patterns
- Final: 9 emoji tokens (🦄🔮🌟🎨🎪🐉💎🎭🦋)

## SIGNIFICANCE
Layer 0 captures the complete semantic structure of the 17,377 files 
before compression. These n-grams represent the fundamental relationships 
that are then encoded into the 8D emoji field through our CFT boundary 
condition system.

The preservation of these patterns (from 14,004 occurrences down to 194) 
demonstrates the hierarchical nature of the codebase structure that gets 
compressed into our final 9 emoji representation.
